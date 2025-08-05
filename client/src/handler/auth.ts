const loginBtn = document.querySelector<HTMLButtonElement>("#login")!;
const accessTokenInput =
  document.querySelector<HTMLInputElement>("#access_token")!;
const accessTokenExpInput =
  document.querySelector<HTMLInputElement>("#access_token_exp")!;
const refreshTokenInput =
  document.querySelector<HTMLInputElement>("#refresh_token")!;
const refreshTokenExpInput =
  document.querySelector<HTMLInputElement>("#refresh_token_exp")!;

type Token = {
  value: string;
  exp: string;
};

type ResponseData = {
  access_token: Token;
  refresh_token: Token;
};

export function authHandler() {
  loginBtn.addEventListener("click", actionLogin);
}

async function actionLogin() {
  const password = "dummy";
  const username = "dummy";

  try {
    const response = await fetch("http://localhost:8000/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username, password }),
    });

    const responseData: ResponseData = await response.json();

    accessTokenInput.value = responseData.access_token.value;
    accessTokenExpInput.value = responseData.access_token.exp;
    refreshTokenInput.value = responseData.refresh_token.value;
    refreshTokenExpInput.value = responseData.refresh_token.exp;
  } catch (err) {
    console.error(err);
  }
}
