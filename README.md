<h1>Elden Ring Mule Copier</h1>

<h4>
    Copy characters from a given save file onto yours*.
</h4>


<image style="margin-top: 60px; margin-bottom: 30px" src="https://i.imgur.com/uKPzDbp.png">

<h3>How to use</h3>
<ul>
<li> GUI </li>
    <ul>
        <li> The UI should be self explanatory. I haven't crash-proofed the UI yet and I am not sure if I have the time to. </li> 
    </ul>
    <li>Command Line</li>
    <ul>
        <li> Invoke the app from your shell in a command line termnal with input and output file as below:</li>
        <li> Input file must be prefixed with `-input`, output must be prefixed with `-output`</li>
        <li> Example on windows: `er_mule_copier.exe -input 'path\to\source\savefile.sl2' -output 'path\to\target\savefile.sl2'`</li>
        <li> On *nix/steamdeck it's: 
        `er_mule_copier -input 'path/to/source/savefile.sl2' -output 'path/to/target/savefile.sl2'`
    </ul>
</ul>

<h3>How to build</h3>

<ul>
    <li>Install the rust SDK on your system: <a>https://www.rust-lang.org/tools/install</a></li>
    <li>Download the project using git or github download</li>
    <li>On the root of the project, issue `cargo build --release`</li>
    </li>The binary will be found in `target/release/`
</ul>

<h3>Installation</h3>
<ul>
    <li>Relaese page: <a href="https://github.com/pubnoconst/er_mule_copier/releases">Releases</a></li>
    <li><b>Linux:</b> Check the release page. </li> 
    <li><b>Windows:</b> Check the realease page. Please install the app outside of `Program Files`, I recommend `Desktop`. See <a href="https://i.imgur.com/tuzkSPC.mp4">Demo</a>.</li>
</ul>

<footer>
<hr>
<b>Disclaimer:</b> *This software comes with no guarantee and liability, back up your save file and use this at your own risk. I hold no liability for any unwanted outcomes of using this software. 
<hr>
<p><b>Acknowledgements: </b> This project would not be possible without <a>https://github.com/BenGrn/EldenRingSaveCopier</a> since I used the magic constants from that repo. I wanted this save copier to work on Linux and decided to port the app.
<hr>
<p>
<b>Attribution: </b><a href="https://www.flaticon.com/free-icons/copy" title="copy icons">Copy icons created by Freepik - Flaticon</a>
</p>


</footer>


