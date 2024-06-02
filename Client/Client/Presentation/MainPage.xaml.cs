namespace Client.Presentation;

public sealed partial class MainPage : Page
{
    public MainPage()
    {
        this.InitializeComponent();
    }
    
    private void Selector_OnSelectionChanged(object sender, SelectionChangedEventArgs e)
    {
        Flyout.Hide();
    }
}
