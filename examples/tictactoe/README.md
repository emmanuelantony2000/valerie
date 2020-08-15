# Tic Tac Toe

This is a port of the [Intro to React](https://reactjs.org/tutorial/tutorial.html).

# Instructions to run

 - Install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
 - Run `wasm-pack build --target web --out-name wasm --out-dir ./static`
 - Inside the `./static` directory make an `index.html` file
 
 ```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Example</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>
```
 
 - Install some static file server like `miniserve` (`cargo install miniserve`)
 - Host the `./static` directory (`miniserve ./static --index index.html`)