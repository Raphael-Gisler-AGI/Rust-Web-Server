function getId (id) { return document.getElementById(id) };

const gameElement = getId("game");
const saveButton = getId("saveButton");

const changedFields = new Set();

const BASE_URL = "http://localhost:7878"

window.onload = async () => {
  const res = await fetch(`${BASE_URL}/game`);
  const data = await res.json();
  displayGame(data);
}

/**
  * @param {boolean[][]} game 
 */
function displayGame(game) {
  let id = 0;
  game.forEach((column) => {
    column.forEach((field) => {
      const fieldElement = document.createElement("div");
      fieldElement.addEventListener('click', onClickField);
      fieldElement.setAttribute("id", id);
      fieldElement.classList.add(field ? 'enabled' : 'disabled');
      gameElement.appendChild(fieldElement);
      id++;
    });
  });
}

/**
  * @param {Event} e 
  */
function onClickField({ target }) {
  const id = target.getAttribute("id");
  if (changedFields.has(id)) {
    changedFields.delete(id);
    target.classList.remove('selected');
  } else {
    changedFields.add(id);
    target.classList.add('selected');
  }
}

saveButton.addEventListener('click', async () => {
  await fetch(`${BASE_URL}/game`, {
    method: "UPDATE",
    body: JSON.stringify(changedFields)
  })
})
