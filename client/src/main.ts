import "./style.css";

import { paymentHandler } from "./handler/payment.ts";
import { authHandler } from "./handler/auth.ts";

authHandler();
paymentHandler();
