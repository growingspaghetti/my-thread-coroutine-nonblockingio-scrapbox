# install typescript

```
npm install -g typescript

ryoji@ubuntu:~/dev/go-chan$ npm install -g typescript
/home/ryoji/node-v12.15.0-linux-x64/bin/tsc -> /home/ryoji/node-v12.15.0-linux-x64/lib/node_modules/typescript/bin/tsc
/home/ryoji/node-v12.15.0-linux-x64/bin/tsserver -> /home/ryoji/node-v12.15.0-linux-x64/lib/node_modules/typescript/bin/tsserver
+ typescript@3.7.5
added 1 package from 1 contributor in 2.445s
```

<pre><font color="#A6E22E"><b>ryoji@ubuntu</b></font>:<font color="#66D9EF"><b>~</b></font>$ tsc
Version 3.7.5
Syntax:   tsc [options] [file...]

Examples: tsc hello.ts
          tsc --outFile file.js file.ts
          tsc @args.txt
          tsc --build tsconfig.json

Options:
 -h, --help                                         Print this message.
 -w, --watch                                        Watch input files.
 --pretty                                           Stylize errors and messages using color and context (experimental).
 --all                                              Show all compiler options.
 -v, --version                                      Print the compiler&apos;s version.
 --init                                             Initializes a TypeScript project and creates a tsconfig.json file....</pre>


# install typescrit formatter
```
npm install -g typescript-formatter
```
<pre><font color="#A6E22E"><b>ryoji@ubuntu</b></font>:<font color="#66D9EF"><b>~</b></font>$ npm install -g typescript-formatter
/home/ryoji/node-v12.15.0-linux-x64/bin/tsfmt -&gt; /home/ryoji/node-v12.15.0-linux-x64/lib/node_modules/typescript-formatter/bin/tsfmt
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> typescript-formatter@7.2.2 requires a peer of typescript@^2.1.6 || &gt;=2.7.0-dev || &gt;=2.8.0-dev || &gt;=2.9.0-dev || &gt;=3.0.0-dev but none is installed. You must install peer dependencies yourself.

+ typescript-formatter@7.2.2
added 9 packages from 9 contributors in 0.928s</pre>

<pre><font color="#A6E22E"><b>ryoji@ubuntu</b></font>:<font color="#66D9EF"><b>/media/dev/go-chan/ts</b></font>$ tsfmt -r src/main8axios2.ts 
replaced src/main8axios2.ts</pre>

# run ts
```
ryoji@ubuntu:~/dev/go-chan/ts$ tsc src/main.ts 
ryoji@ubuntu:~/dev/go-chan/ts$ node src/main.ts 
Hello
And Welcome
To Async Await Using TypeScript
```
<pre><font color="#A6E22E"><b>ryoji@ubuntu</b></font>:<font color="#66D9EF"><b>/media/dev/go-chan/ts</b></font>$ tsc &amp;&amp; node src/main2.js
Hello
And Welcome
To Async Await Using TypeScript

</pre>

# install tslint
```
yarn global add tslint typescript
```

# async
```
cd ts;
tsc
node lib/main.js
```

<pre><font color="#A6E22E"><b>ryoji@ubuntu</b></font>:<font color="#66D9EF"><b>/media/dev/go-chan/ts</b></font>$ node lib/main4.js
Promise { <font color="#A1EFE4">&lt;pending&gt;</font> }
OK
</pre>

<pre><font color="#A6E22E"><b>ryoji@ubuntu</b></font>:<font color="#66D9EF"><b>/media/dev/go-chan/ts</b></font>$ tsc &amp;&amp; node src/main3.js
Gilad
Error: Error
    at /media/dev/go-chan/ts/src/main3.js:50:19
    at step (/media/dev/go-chan/ts/src/main3.js:32:23)
    at Object.next (/media/dev/go-chan/ts/src/main3.js:13:53)
    at /media/dev/go-chan/ts/src/main3.js:7:71
    at new Promise (&lt;anonymous&gt;)
    at __awaiter (/media/dev/go-chan/ts/src/main3.js:3:12)
    at giladErr (/media/dev/go-chan/ts/src/main3.js:48:12)
    at Object.&lt;anonymous&gt; (/media/dev/go-chan/ts/src/main3.js:54:1)
<font color="#75715E">    at Module._compile (internal/modules/cjs/loader.js:955:30)</font>
<font color="#75715E">    at Object.Module._extensions..js (internal/modules/cjs/loader.js:991:10)</font>
</pre>

# axios (async io) library
```
npm install axios
```
<pre><font color="#A6E22E"><b>ryoji@ubuntu</b></font>:<font color="#66D9EF"><b>/media/dev/go-chan/ts</b></font>$ npm install axios
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> <font color="#AE81FF">saveError</font> ENOENT: no such file or directory, open &apos;/media/dev/go-chan/ts/package.json&apos;
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> <font color="#AE81FF">enoent</font> ENOENT: no such file or directory, open &apos;/media/dev/go-chan/ts/package.json&apos;
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> ts No description
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> ts No repository field.
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> ts No README data
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> ts No license field.

+ axios@0.19.2
added 4 packages from 7 contributors and audited 7 packages in 1.368s
found <font color="#A6E22E">0</font> vulnerabilities
</pre>

<pre><font color="#A6E22E"><b>ryoji@ubuntu</b></font>:<font color="#66D9EF"><b>/media/dev/go-chan/ts</b></font>$ npm install @types/node --save-dev
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> ts No description
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> ts No repository field.
<span style="background-color:#272822"><font color="#F8F8F2">npm</font></span> <span style="background-color:#F4BF75"><font color="#272822">WARN</font></span> ts No license field.

+ @types/node@13.7.1
added 1 package from 41 contributors, updated 4 packages and audited 5 packages in 1.351s
found <font color="#A6E22E">0</font> vulnerabilities

</pre>