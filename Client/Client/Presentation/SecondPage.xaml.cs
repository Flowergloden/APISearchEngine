namespace Client.Presentation;

public sealed partial class SecondPage : Page
{
    public SecondPage()
    {
        this.InitializeComponent();
    }
}

public record Entity(IImmutableList<EntityTemplate> Result);

public record EntityTemplate(string Kind, string Path, string Source);
