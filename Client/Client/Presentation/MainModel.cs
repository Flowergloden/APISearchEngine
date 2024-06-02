using System.Text.Json;

namespace Client.Presentation;

public static class ExtendString
{
    public static string HtmlUnEscape(this string value)
    {
        return value.Replace("&lt;", "<").Replace("&gt;", ">");
    }
    
    public static string HtmlEscape(this string value)
    {
        return value.Replace("<", "&lt;").Replace(">", "&gt;");
    }
}

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
    
    public IState<string> SearchMode => State<string>.Value(this, () => "in name");
    
    public IState<string> SearchContext => State<string>.Value(this, () => string.Empty);
    
    public string[] Modes = ["in name", "in parameter", "in return type"];
    
    private const string ItemPropertyName = "items";
    
    private async ValueTask<IImmutableList<EntityTemplate>> GetResult()
    {
        var ans = ImmutableList.Create<EntityTemplate>();
        
        var mode = await SearchMode switch
        {
            "in name" => "name",
            "in parameter" => "para",
            "in return type" => "rt",
            _ => "name",
        };
        var context = await SearchContext;
        if (context == "") return ans;
        
        var uri = Dir + '/' + mode! + '/' + context!.HtmlEscape();
        var msg = await App.HttpClient.GetAsync(uri);
        var res = await JsonDocument.ParseAsync(await msg.Content.ReadAsStreamAsync());
        var root = res.RootElement;
        var items = root.GetProperty(ItemPropertyName);
        foreach (var item in items.EnumerateArray())
        {
            var kind = item.GetProperty("kind").GetString()?.HtmlUnEscape();
            var path = item.GetProperty("path").GetString()?.HtmlUnEscape();
            var source = item.GetProperty("source").GetString()?.HtmlUnEscape();
            
            ans = ans.Add(new(kind!, path!, source!));
        }
        
        return ans;
    }
    
    public async Task GoToSecond()
    {
        var name = await SearchContext;
        var result = await GetResult();
        await _navigator.NavigateViewModelAsync<SecondModel>(this, data: new Entity(result));
    }
}
