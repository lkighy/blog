import hljs from 'highlight.js';

import "highlight.js/scss/vs2015.scss";

let md = require('markdown-it')({
    html: false,
    xhtmlOut: false,
    breaks: false,
    langPrefix: 'language-',
    linkify: true,
    typographer: true,
    highlight: function(str, lang) {
        var esc = md.utils.escapeHtml;

        if (lang && hljs.getLanguage(lang)) {
            try {
                return '<pre class="hljs"><code>' +
                    hljs.highlight(lang, str, true).value +
                    '</code></pre>';
            } catch (__) { }
        }

        return '<pre class="hljs"><code>' + esc(str) + '</code></pre>';
    }
});

// 用于markdown-it markdown解析器的插件，添加了表情符号和表情符号语法支持。
let emoji = require("markdown-it-emoji");
let sub = require("markdown-it-sub");
let sup = require("markdown-it-sup");
let container = require("markdown-it-container");
let deflist = require("markdown-it-deflist");
let abbr = require("markdown-it-abbr");
let footnote = require("markdown-it-footnote");
let ins = require("markdown-it-ins");
let mark = require("markdown-it-mark");
let taskLists = require("markdown-it-task-lists");
let katex = require("markdown-it-katex");

md.use(emoji);
md.use(sub);
md.use(sup);
md.use(container);
md.use(deflist);
md.use(abbr);
md.use(footnote);
md.use(ins);
md.use(mark);
md.use(taskLists);
md.use(katex);

export default md;