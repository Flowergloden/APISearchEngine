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
    
    public IState<string> SearchContext => State<string>.Value(this, () => string.Empty);
    
    public IListFeed<EntityTemplate> Result => ListFeed<EntityTemplate>.Async(async ct => await GetResult());
    
    private async Task<IImmutableList<EntityTemplate>> GetResult()
    {
        return ImmutableList.Create<EntityTemplate>().Add(new("TestKind", "TestPath", "TestSource"));
    }
    
    public async Task GoToSecond()
    {
        var name = await SearchContext;
        var result = await Result;
        await _navigator.NavigateViewModelAsync<SecondModel>(this, data: new Entity(result));
    }
}
