<h1>Elden Ring Mule Copier</h1>

<h4>
Copy characters from one Elden Ring save file into another.
</h4>

<h3>Overview</h3>

This is a small, interactive command-line tool that lets you copy a character slot from one save file (`.sl2`/`.den`) into another.
It is designed to be simple, dependency-light, and portable..

<h3>Installation</h3>

<ul>
<li>Prebuilt binaries are available on the <a href="https://github.com/pubnoconst/er_mule_copier/releases">releases page</a></li>
<li>On Wndows you should just be able to double click to start the program</li>
<li>Linux releases are statically linked and should run on most systems but you have to put the file into your $PATH and invoke it from your terminal.</li>
</ul>

There is no GUI and no automatic backup. You are expected to back up your save files yourself.

<h3>How to use</h3>

<ul>
<li>Run the executable from a terminal (PowerShell, CMD, bash, etc.).</li>
<li>The program will:
  <ul>
    <li>Welcome you and display a warning.</li>
    <li>Ask you to drop the <b>source</b> save file into the terminal and press Enter.</li>
    <li>Ask you to drop the <b>target</b> save file into the terminal and press Enter.</li>
    <li>Display the character slots for both saves.</li>
    <li>Prompt you to choose a source slot and a target slot.</li>
    <li>Overwrite the target save file.</li>
  </ul>
</li>
<li>You can repeat the process or exit explicitly. Ctrl+C always works.</li>
</ul>

<p>
On Windows, dragging a file into the console will paste its path automatically.
</p>

<h3>How to build</h3>

<ul>
<li>Install Rust: <a href="https://www.rust-lang.org/tools/install">https://www.rust-lang.org/tools/install</a></li>
<li>Clone or download the repository.</li>
<li>From the project root, run:</li>
</ul>

<pre>
`cargo build --release`
</pre>

<p>
The binary will be located in <code>target/release/</code>.
</p>

<footer>
<hr>
<p><b>Disclaimer:</b> This software comes with no guarantees. It will overwrite save files. Back up your saves before using it. You assume all risk.</p>

<p><b>Acknowledgements:</b> This project is based on the magic constants discovered in <a href="https://github.com/BenGrn/EldenRingSaveCopier">BenGrn/EldenRingSaveCopier</a>. This is a Linux- and CLI-focused reimplementation.</p>
