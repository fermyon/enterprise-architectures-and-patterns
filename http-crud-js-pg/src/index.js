import { Router } from "@fermyon/spin-sdk";
import { loadConfig } from "./config";
import { deleteItemById, deleteManyItems, getAllItems, getItemById, createItem, updateItemById, notFound } from "./handlers";

const router = Router();

router.get("/items", loadConfig, ({ config }) => getAllItems(config));
router.get("/items/:id", loadConfig, ({ params, config }) => getItemById(config, params.id));
router.post("/items", loadConfig, ({ config }, requestBody, { baseUrl }) => createItem(config, baseUrl, requestBody));
router.put("/items/:id", loadConfig, ({ config, params }, requestBody, { baseUrl }) => updateItemById(config, baseUrl, params.id, requestBody));
router.delete("/items", loadConfig, ({ config }, payload) => deleteManyItems(config, payload));
router.delete("/items/:id", loadConfig, ({ params, config }) => deleteItemById(config, params.id));
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
