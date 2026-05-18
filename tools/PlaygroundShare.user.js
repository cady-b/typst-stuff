// ==UserScript==
// @author      Liara
// @name        PlaygroundShare
// @version     2026-05-18 03:25
// @match       https://typst.app/play
//
// @grant       GM_setClipboard
// @run-at      document-end
// @namespace   Violentmonkey Scripts
// ==/UserScript==

(function() {
  'use strict';

  const shareParam = "state";
  const shareLink = `https://typst.app/play?${shareParam}=`
  const editorQuery = "#app-root .cm-editor .cm-content";

  let foundEditor = false;
  let foundShareButton = false;

  const editorHook = (editor) => {
    // guard against duplicate calls
    if (foundEditor === true) { return; }
    foundEditor = true;

    let share = new URLSearchParams(window.location.search).get(shareParam);

    // early return if no body was requested by the user
    if (!share) { return; }

    // Remove search params so the user doesn't get confused
    window.history.replaceState(null, "", window.location.href.split("?")[0])

    // try to decode the value
    let dbody = null;
    try {
      dbody = atob(share); // docs say this can fail; in practice it rarely does even with random input
    } catch (e) {
      alert("PlaygroundShare: Unable to decode URL :(\nMore info in the console")
      console.log(e);
    }

    // early return if decoding failed
    if (!dbody) { return; }

    // inject body

    let last = "";
    let stable = 0;

    const finishLoading = () => {
      // await the "editable" signal
      if (editor.getAttribute("contenteditable") !== "true") { return requestAnimationFrame(finishLoading); }

      let now = editor.innerText;
      if (now === last) { stable++; }
      last = now;

      // await stability
      if (stable <3) { return requestAnimationFrame(finishLoading); }

      // inject text
      editor.innerText = dbody;
    };

    finishLoading();
  }

  const shareHook = (share) => {
    // Overwrite the onclick event with a handler that encodes the editor state
    share.onclick = (event) => {
      event.stopImmediatePropagation();

      let editor = document.querySelector(editorQuery);
      GM_setClipboard(shareLink + btoa(editor.innerText));
    }
  }

  const documentObserver = new MutationObserver(records => {
    for (const record of records) {
      for (const node of record.addedNodes) {
        if (!(node instanceof HTMLElement)) continue;

        if (!foundEditor) {
          const editor = node.matches?.(editorQuery) ? node : node.querySelector(editorQuery);
          if (editor) { editorHook(editor) }
        }

        // Find the share button (sadly we have to keep the observer running because of dynamic rendering shenanigans)
        const share = node.querySelectorAll("div[role=toolbar] button")?.values().find(v => v.innerText == "Share");
        if (share) { shareHook(share) }
      }
    }
  })

  // Start observing 👁️
  documentObserver.observe(document.body, {
    childList: true,
    subtree: true,
  })
})();
