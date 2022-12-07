const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let counterButton;
let counterResult;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function counter() {
  //Counter Button Click
  counterResult.textContent = await invoke("add_count", { num: 1 });
}

window.addEventListener("DOMContentLoaded", () => {
  //Get Elements
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");

  counterButton = document.querySelector("#counter-button");
  counterResult = document.querySelector("#counter-result");

  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());

  document
    .querySelector("#counter-button")
    .addEventListener("click", () => counter());
});
