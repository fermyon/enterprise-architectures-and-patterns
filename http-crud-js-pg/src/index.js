import { Router } from "@fermyon/spin-sdk";
import { loadConfig } from "./config";
import { deleteItemById, deleteManyItems, getAllItems, getItemById, createItem, updateItemById, notFound } from "./handlers";

const router = Router();

router.get("/items", loadConfig, ({ config }, _req, res) => getAllItems(config, res));
router.get("/items/:id", loadConfig, ({ params, config }, _req, res) => getItemById(config, params.id, res));
router.post("/items", loadConfig, async ({ config }, req, res, { baseUrl }) => createItem(config, baseUrl, await req.arrayBuffer(), res));
router.put("/items/:id", loadConfig, async ({ config, params }, req, res, { baseUrl }) => updateItemById(config, baseUrl, params.id, await req.arrayBuffer(), res));
router.delete("/items", loadConfig, async ({ config }, req, res) => deleteManyItems(config, await req.arrayBuffer(), res));
router.delete("/items/:id", loadConfig, ({ params, config }, _req, res) => deleteItemById(config, params.id, res));
router.all("*", () => notFound("Endpoint not found"));

export async function handler(request, res) {

    const fullUrl = request.headers.get("spin-full-url");
    const path = request.headers.get("spin-path-info");
    const baseUrl = fullUrl.substr(0, fullUrl.indexOf(path))

    return await router.handleRequest(request, res, {
        baseUrl,
        fullUrl,
        path
    });
}
