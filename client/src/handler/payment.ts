const paymentBtn = document.querySelector<HTMLButtonElement>("#payment")!;
const priceEl = document.querySelector<HTMLInputElement>("#price")!;

export function paymentHandler() {
  paymentBtn.addEventListener("click", actionPayment);
}

async function actionPayment() {
  const price = Number(priceEl.value);
  if (price <= 0) {
    console.error("1円以上で選択してください");
    return;
  }

  try {
    await fetch("http://localhost:8000/payment/send", {
      method: "POST",
      body: JSON.stringify({ price }),
    });
  } catch (err) {
    console.error(err);
  }
}
