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
    
    public async Task GoToSecond()
    {
        var name = await SearchContext;
        await _navigator.NavigateViewModelAsync<SecondModel>(this, data: new Entity(new List<EntityTemplate>()
        {
            new(name!, name!, name!)
        }));
    }
}
