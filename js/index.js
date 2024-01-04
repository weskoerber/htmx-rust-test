import * as greet from "./modules/hello.js"
import * as list from "./modules/list.js"

window.addEventListener("load", (_) => {
  greet.hello();

  const items = document.querySelector("#list");
  document.querySelector("#list-add").addEventListener("click", (_) => list.add(items));
  document.querySelector("#list-remove").addEventListener("click", (_) => list.remove(items));
});
