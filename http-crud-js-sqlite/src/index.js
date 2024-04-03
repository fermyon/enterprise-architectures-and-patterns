import { Router } from "@fermyon/spin-sdk";
import { deleteItemById, deleteManyItems, getAllItems, getItemById, createItem, updateItemById, notFound } from "./handlers";

const router = Router();

router.get("/items", () => getAllItems());
router.get("/items/:id", ({ params }) => getItemById(params.id));
router.post("/items", ({ }, requestBody, { baseUrl }) => createItem(baseUrl, requestBody));
router.put("/items/:id", ({ params }, requestBody, { baseUrl }) => updateItemById(baseUrl, params.id, requestBody));
router.delete("/items", ({ }, payload) => deleteManyItems(payload));
router.delete("/items/:id", ({ params }) => deleteItemById(params.id));
router.all("*", () => notFound("Endpoint not found"));

export async function handleRequest(request) {
    let fullUrl = request.headers["spin-full-url"];
    let path = request.headers["spin-path-info"];
    let baseUrl = fullUrl.substr(0, fullUrl.indexOf(path))

    return await router.handleRequest(request, request.body, {
        baseUrl,
        fullUrl,
        path
    });
}
