import { Router } from "@fermyon/spin-sdk";
import { deleteItemById, deleteManyItems, getAllItems, getItemById, createItem, updateItemById, notFound } from "./handlers";

const router = Router();

router.get("/items", (_metadata, _req, res) => getAllItems(res));
router.get("/items/:id", ({ params }, _req, res) => getItemById(params.id, res));
router.post("/items", async ({ }, req, res, { baseUrl }) => createItem(baseUrl, await req.arrayBuffer(), res));
router.put("/items/:id", async ({ params }, req, res, { baseUrl }) => updateItemById(baseUrl, params.id, await req.arrayBuffer(), res));
router.delete("/items", async ({ }, req, res) => deleteManyItems(await req.arrayBuffer(), res));
router.delete("/items/:id", ({ params }, _req, res) => deleteItemById(params.id, res));
router.all("*", (_metadata, _req, res) => notFound("Endpoint not found", res));

export async function handler(req, res) {
    const fullUrl = req.headers.get("spin-full-url");
    const path = req.headers.get("spin-path-info");
    const baseUrl = fullUrl.substr(0, fullUrl.indexOf(path))
    console.info(`Processing incoming ${req.method} request for ${path}`);
    return await router.handleRequest(req, res, {
        baseUrl,
        fullUrl,
        path
    });
}
