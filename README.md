<h1>Elden Ring Mule Copier</h1>

<h4>
    Copy characters from a given save file onto yours*.
</h4>


<image style="margin-top: 60px; margin-bottom: 30px" src="https://i.imgur.com/2A7XaUx.png">

<h3>How to use</h3>
<ul>
<li> GUI </li>
    <ul>
        <li> The UI should be self explanatory. I haven't crash-proofed the UI yet and I am not sure if I have the time to. </li> 
    </ul>
    <li>Command Line</li>
    <ul>
        <li> Invoke the app from your shell in a command line termnal with input and output file as below:</li>
        <li> Input file must be prefixed with `-s`, output must be prefixed with `-t`</li>
        <li> Example on windows: `er_mule_copier.exe -s 'path\to\source\savefile.sl2' -t 'path\to\target\savefile.sl2'`.</li>
        <li> On *nix/steamdeck it's: 
        `er_mule_copier -s 'path/to/source/savefile.sl2' -t 'path/to/target/savefile.sl2'`.
    </ul>
</ul>

<h3>How to build</h3>

<ul>
    <li>Install the rust SDK on your system: <a>https://www.rust-lang.org/tools/install</a></li>
    <li>Download the project using git or github download.</li>
    <li>On the root of the project, issue `cargo build --release --bin gui` or `cargo build --release --bin cli` depending on whether you want the CLI or the GUI frontend.</li>
    </li>The binary will be found in `target/release/` named `gui` or `cli` depending on what you build.
</ul>

<h3>Installation</h3>
<ul>
    See the <a href="https://github.com/pubnoconst/er_mule_copier/releases">releases page</a>.
</ul>

<footer>
<hr>
<p><b>Disclaimer:</b> *This software comes with no guarantee and liability, back up your save file and use this at your own risk. I hold no liability for any unwanted outcomes of using this software.</p>

<p><b>Acknowledgements: </b> This project would not be possible without <a href="https://github.com/BenGrn/EldenRingSaveCopier"> BenGrn/EldenRingSaveCopier</a> since I used the magic constants from that repo. I wanted this save copier to work on Linux and decided to port the app.
<p><b>Attribution: </b><a target="_blank" href="https://icons8.com/icon/8IL0nIbrmB7p/one-page-up">One Page Up</a> icon by <a target="_blank" href="https://icons8.com">Icons8</a></p>
</footer>

