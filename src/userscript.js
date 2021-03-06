// ==UserScript==
// @name        startpage
// @namespace   Violentmonkey Scripts
// @grant       GM_openInTab
// @version     1.1
// @author      simon-wf
// @description shortcut to open startpage
// ==/UserScript==

const KEY_URL = null;
document.addEventListener("keydown", (event) => {
    ;;KEY_URL;;
    if (event.ctrlKey) {
        if (event.key == KEY) {
            event.stopPropagation();
            event.preventDefault();
            GM_openInTab(URL);
        }
    }
}, false);
