const paymentBtn = document.querySelector<HTMLButtonElement>("#payment")!;
const priceEl = document.querySelector<HTMLInputElement>("#price")!;
const accessTokenInput =
  document.querySelector<HTMLInputElement>("#access_token")!;

export function paymentHandler() {
  paymentBtn.addEventListener("click", actionPayment);
}

async function actionPayment() {
  const price = Number(priceEl.value);
  if (price <= 0) {
    alert("1円以上で選択してください");
    return;
  }

  const accessToken = accessTokenInput.value;
  if (!accessToken) {
    alert("ログインしてください");
    return;
  }

  try {
    await fetch("http://localhost:8000/payment/send", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${accessToken}`,
      },
      body: JSON.stringify({ price }),
    });
  } catch (err) {
    console.error(err);
  }
}
