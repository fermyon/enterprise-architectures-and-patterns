import { HandleRequest, HttpRequest, HttpResponse, Router } from "@fermyon/spin-sdk"
import { Api } from "./api";

let router = Router();
export const handleRequest: HandleRequest = async function (request: HttpRequest): Promise<HttpResponse> {
  const api = new Api();
  return api.handleRequest(request);
}
