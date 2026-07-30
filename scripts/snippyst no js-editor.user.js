// ==UserScript==
// @author      Cady
// @name        Snippyst no js-editor
// @version     2026-07-01
// @match       *://snippyst.com/snippets/*
// @grant       none
// @run-at      document-end
// ==/UserScript==

(function() {
  'use strict';

  let path = new URL(window.location).pathname;
  fetch("https://api.snippyst.com/v1" + path, {
    "referrer": "https://snippyst.com/",
    "method": "GET",
  }).then(response => response.blob())
    .then(blob     => blob.text())
    .then(snippet  => {
      let pre = document.createElement("pre");
      let code = document.createElement("code");
      code.textContent = JSON.parse(snippet).content;
      pre.appendChild(code);

      let editor = document.querySelector('.md\\:block section');
      editor.replaceWith(pre);
  });
})();
