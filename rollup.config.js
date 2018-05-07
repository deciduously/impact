import babel from "rollup-plugin-babel"
import uglify from "rollup-plugin-uglify"

export default {
    input: './target/deploy/impact.js',
    output: {
        name: 'impact',
        file: './release/impact.js',
        format: 'es',
    },
    plugins: [
        babel({
            exclude: 'node_modules/**'
        }),
        uglify()
    ]
};
