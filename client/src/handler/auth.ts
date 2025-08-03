const loginBtn = document.querySelector<HTMLButtonElement>("#login")!;

export function authHandler() {
  loginBtn.addEventListener("click", actionLogin);
}

async function actionLogin() {
  const password = "dummy";
  const username = "dummy";

  try {
    await fetch("http://localhost:8000/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username, password }),
    });
  } catch (err) {
    console.error(err);
  }
}
