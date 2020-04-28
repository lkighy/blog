const path = require("path");
const htmlWebpackPlugin = require("html-webpack-plugin");
const MiniCSSExtractPlugin = require('mini-css-extract-plugin');
const VueLoaderPlugin = require('vue-loader/lib/plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

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
            use: [
                MiniCSSExtractPlugin.loader,
                "css-loader"
            ]
        }, {
            test: /\.scss$/,
            use: [
                MiniCSSExtractPlugin.loader,
                "css-loader",
                "sass-loader"
            ]
        }, {
            test: /\.(png|jpg|jpeg|gif|eot|ttf|woff|woff2|svg|svgz)(\?.+)?$/,
            use: [{
                loader: 'file-loader',
                options: {
                    name: "static/[name].[ext]",
                    limit: 10000
                }
            }]
        }]
    },
    plugins: [
        new MiniCSSExtractPlugin({
            filename: 'css/[name].css',
            chunkFilename: 'css/[id].css'
        }),
        new VueLoaderPlugin(),
        new htmlWebpackPlugin({
            template: "src/index.html"
        }),
        new CleanWebpackPlugin()
    ],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "src")
        },
        extensions: ['.js', '.vue', '.json', '.scss', '.css']
    }
}