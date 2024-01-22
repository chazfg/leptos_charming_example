# Leptos and Charming Example
Demo of Leptos and Charming charts where Apache eCharts is added to final build through Trunk. Make sure you have Trunk and node installed, along with npm. \
[Live Demo](https://basic-graph.keepsafe.dev/) \
[Leptos](https://leptos.dev/) \
[Charming](https://github.com/yuankunzhang/charming) \
[Trunk](https://trunkrs.dev/) \
[eCharts](https://echarts.apache.org/en/index.html) 
## Simple Way to Get Running
Clone this repo locally. Enter project directory. Run "npm install" to install node deps (echarts and tailwindcss). Run "trunk serve". The chart should display at localhost:8080
## Installing eCharts
  - npm install echarts
  - Link eCharts to Trunk by referencing it in the index.html
    ```
    <!DOCTYPE html>
    <html>
      <head></head>
      <script data-trunk src="node_modules/echarts/dist/echarts.js"/>
      <title>Leptos and Charming</title>
      <body></body>
    </html>
    ```
    The src should be set equal to the path to echarts in the node_modules directory. Depends on your file structure.

## Installing Tailwindcss
  - npm install tailwindcss
  - create file "tailwind.css", should have the following lines.
    ```
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
    ```
  - Add to index.html, should look like this.
    ```
    <!DOCTYPE html>
    <html>
      <head></head>
      <script data-trunk src="node_modules/echarts/dist/echarts.js"/>
      <link data-trunk rel="tailwind-css" href="/tailwind.css" />
      <title>Leptos and Charming</title>
      <body></body>
    </html>
    ```
  - Add tailwind to Trunk.toml. This adds a tailwind step to the trunk build process. Looks like:
    ```
    [tools]
    tailwindcss = "*"
    ```
