using System.Text.Json;

namespace Client.Presentation;

public partial record MainModel
{
    private INavigator _navigator;
    
    public MainModel(
        IOptions<AppConfig> appInfo,
        INavigator navigator)
    {
        _navigator = navigator;
        Title = "API-SearchEngine";
    }
    
    public string? Title { get; }
    
    private const string Dir = "search";
    
    public IState<string> SearchMode => State<string>.Value(this, () => "name");
    
    public IState<string> SearchContext => State<string>.Value(this, () => string.Empty);
    
    private const string ItemPropertyName = "items";
    
    // public IListState<EntityTemplate> Result => ListState<EntityTemplate>.Async(this, async _ => await GetResult());
    
    private async ValueTask<IImmutableList<EntityTemplate>> GetResult()
    {
        var ans = ImmutableList.Create<EntityTemplate>();
        
        var mode = await SearchMode;
        var context = await SearchContext;
        // if (context == "") return ans;
        
        var uri = Dir + '/' + mode! + '/' + context!;
        Console.WriteLine(uri);
        var msg = await App.HttpClient.GetAsync(uri);
        var res = await JsonDocument.ParseAsync(await msg.Content.ReadAsStreamAsync());
        var root = res.RootElement;
        var items = root.GetProperty(ItemPropertyName);
        foreach (var item in items.EnumerateArray())
        {
            var kind = item.GetProperty("kind").GetString();
            var path = item.GetProperty("path").GetString();
            var source = item.GetProperty("source").GetString();
            ans = ans.Add(new(kind!, path!, source!));
        }
        
        return ans;
    }
    
    public async Task GoToSecond()
    {
        var name = await SearchContext;
        Console.WriteLine(name);
        var result = await GetResult();
        await _navigator.NavigateViewModelAsync<SecondModel>(this, data: new Entity(result));
    }
}
