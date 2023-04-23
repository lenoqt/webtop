import { h, Component, render } from "preact";

document.addEventListener("DOMContentLoaded", () => {
  setInterval(async () => {
    let response = await fetch("/api/cpu");
    if (response.status !== 200) {
      throw new Error(`HTTP Error! Status: ${response.status}`);
    }
    let json = await response.json();

    const app = h('pre', null, JSON.stringify(json, null, 2));
    render(app, document.body)
  }, 1000);
  document.body.textContent = "Your content is mine";
});
