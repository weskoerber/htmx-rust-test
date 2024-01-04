/**
  * @param {Element} list
  */
function add(list) {
  const item = document.createElement("li");
  item.innerHTML = list.children.length;
  list.appendChild(item);
}

/**
  * @param {Element} list
  */
function remove(list) {
  if (list.children.length) {
    list.removeChild(list.lastElementChild);
  }
}

export { add, remove };
