import { HttpRequest, HttpResponse, Router, routerType } from "@fermyon/spin-sdk";
import { getAllItems, getCustomerById, getCustomerCount, getTopCustomers } from "./handlers";

export class Api {
    router: routerType

    constructor() {
        this.router = Router()
        this.router.get("/customers/count", () => getCustomerCount())
        this.router.get("/customers/top/:limit", ({ params }) => getTopCustomers(params.limit))
        this.router.get("/customers/items", () => getAllItems())
        this.router.get("/customers/items/:id", ({ params }) => getCustomerById(params.id))
    }

    async handleRequest(r: HttpRequest): Promise<HttpResponse> {
        return await this.router.handleRequest(r);
    }
}