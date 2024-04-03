import { Router } from "@fermyon/spin-sdk";
import { deleteItemById, deleteManyItems, getAllItems, getItemById, createItem, updateItemById, notFound } from "./handlers";
import { seed } from "./seed";

const router = Router();

router.get("/items", () => getAllItems());
router.get("/items/:id", ({ params }) => getItemById(params.id));
router.post("/items", ({ }, requestBody, { baseUrl }) => createItem(baseUrl, requestBody));
router.put("/items/:id", ({ params }, requestBody, { baseUrl }) => updateItemById(baseUrl, params.id, requestBody));
router.delete("/items/batch", ({ }, payload) => deleteManyItems(payload));
router.delete("/items/:id", ({ params }) => deleteItemById(params.id));
router.all("*", () => notFound("Endpoint not found"));

export async function handleRequest(request) {
    // todo: refactor this to use migration.sql when running locally or in cloud
    // for SpinKube and Fermyon Platform for Kubernetes the corresponding
    // database needs to be pre-configured upon deployment
    seed();
    let fullUrl = request.headers["spin-full-url"];
    let path = request.headers["spin-path-info"];
    let baseUrl = fullUrl.substr(0, fullUrl.indexOf(path))

    return await router.handleRequest(request, request.body, {
        baseUrl,
        fullUrl,
        path
    });
}
