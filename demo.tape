Output "media/demo.gif"
# Output "media/frames/"
Set Shell zsh
Set FontSize 16
Set Width 1280
Set Height 720
Set FontFamily "jetbrainsmono nfm"
Set Theme "Catppuccin Macchiato"
Set WindowBar Colorful
Set Framerate 60


Hide
  Type "tmux -f /dev/null -L test new-session -- fish" Enter
  Type "tmux set status && \" Enter
  Type 'tmux setw pane-border-style "fg=0" && \' Enter
  Type 'tmux setw pane-active-border-style "fg=0"' Enter
  Sleep 0.5
  Ctrl+L
  Sleep 1
Show


Type "cargo run -q"
Enter 1
Sleep 2s
Ctrl+B
Type "["
Up 100
Sleep 10s

Hide
  Ctrl+C