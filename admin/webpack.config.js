const path = require("path");
const htmlWebpackPlugin = require("html-webpack-plugin");
const VueLoaderPlugin      = require('vue-loader/lib/plugin');

module.exports = {
    entry: {
        index: path.resolve(__dirname, "src/index.js"),
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        publicPath: "/",
        filename: "js/[name].bundle.js",
        chunkFilename: "js/[id].chunck.js"
    },
    module: {
        rules: [{
            test: /\.vue$/,
            use: ["vue-loader"]
        }, {
            test: /\.js$/,
            use: ["babel-loader"],
            exclude: /node_modules/
        }, {
            test: /\.css$/,
            use: ["vue-style-loader", "css-loader"]
        }, {
            test: /\.scss$/,
            use: ["vue-style-loader", "css-loader", "sass-loader"]
        }, {
            test: /\.(png|jpg|jpeg|gif|eot|ttf|woff|woff2|svg|svgz)(\?.+)?$/,
            use: [{
                loader: 'url-loader',
                options: {
                    limit: 10000
                }
            }]
        }]
    },
    plugins: [
        new VueLoaderPlugin(),
        new htmlWebpackPlugin({
            template: "src/index.html"
        })
    ],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "src")
        },
        extensions: ['.js', '.vue', '.json', '.scss', '.css']
    },
    devServer: {
        host: '127.0.0.1',
        port: 8081,
        proxy: {
            "/api/": {
                target: "http://localhost:8088",
                changeOrigin: true,
                pathRewrite: {
                    "^/api": ""
                }
            }
        }
    }
}