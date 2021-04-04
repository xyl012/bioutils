# Bioutils

## Simple Biological Utilities with Rust's [u8]

<p>Bioutils provides simple biological utilities including: 
    <ul>complete iupac and quality character sets</ul>
    <ul> Functions to check sequence validity and content (palindromes too!)</ul>
    <ul> Functions to download human and mouse Gencode reference files</ul>
    <ul> Functions to download fastq files</ul>
    <ul> Functions to replace N or gaps with pseudorandom nucleotides</ul>
    <ul> Functions to create new random IUPAC sequences</ul>
</p>

<p> Please take a look at the align example to get a full practical walkthrough!</p>

<li>Character sets include punctuation, are subdivided, and implemented in Rust's [u8] rather than bitset</li>
<li>Implementations are centered around [u8], although character sets are also provided as [&amp;str], hashset u8 and hashset &amp;str.
Check back as more functionality gets added!</li>
</ul>
<h2 id="quick-start" class="section-header"><a href="#quick-start">Quick Start</a></h2>
<div class="example-wrap"><pre class="rust rust-example-rendered">
</pre></div>
</div><h2 id='modules' class='section-header'><a href="#modules">Modules</a></h2>
<table><tr class='module-item'><td><a class="mod" href="charsets/index.html" title='bioutils::charsets mod'>charsets</a></td><td class='docblock-short'><p>Numerous IUPAC character sets to either use directly or create your own mix and match</p></td></tr><tr class='module-item'><td><a class="mod" href="files/index.html" title='bioutils::files mod'>files</a></td><td class='docblock-short'><p>Download human and mouse Gencode references, download fastq sample files</p></td></tr><tr class='module-item'><td><a class="mod" href="references/index.html" title='bioutils::references mod'>references</a></td><td class='docblock-short'><p>Currently includes human NCBI gencode GRCh38. Automatically downloads the latest version of user's choice.</p>
</td></tr><tr class='module-item'><td><a class="mod" href="utils/index.html" title='bioutils::utils mod'>utils</a></td><td class='docblock-short'><p>Functions for sequence checks, pseudorandom replacement of N or gaps, and functions to create new pseudoranndom sequences</p></td></tr></table></section><section id="search" class="content hidden"></section><section class="footer"></section>