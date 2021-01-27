const path = require('path')

module.exports = {
    watch: true,
    mode: "development",
    entry: './src/mew.js',
    output: {
        filename: "mew.js",
        path: path.resolve("./dist")
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /(node_modules|bower_components)/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['@babel/preset-env']
                    }
                }
            },
        ]
    }
};
