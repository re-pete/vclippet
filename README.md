# General
This project will be a TUI/CLI interface to using ffmpeg to extract clips from a movie. 

## Dependencies
 - AI told me that ratatui, crossterm, and clap are useful libraries for this project
 - ffmpeg must be installed and available on the path. 

# What it actually does 
At the end of the day, all this thing is going to do is run something like:

```ffmpeg -ss [start] -to [end] -i input.mp4 -c copy -avoid_negative_ts make_zero output.mp4```

# TODO
 - Warn user that the beginning of the clip could be off by a few seconds
 - Look into if I can find exactly how far off it will be 
 - Maybe I could load an ffmpeg library directly? rather than use its CLI?

# Notes
 - always use -avoid_negative_ts make_zero on all ffmpeg calls to "snap to" the closest keyframe
   this will cause the start time to be earlier than expected, but stop any "smearing" at the start
 - maybe I could figure how to encode a new keyframe exactly where the user selected? 