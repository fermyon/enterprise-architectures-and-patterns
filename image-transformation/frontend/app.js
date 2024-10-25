const filter = document.getElementById("filter");
const fileInput = document.getElementById("image");

const containerOriginalImage = document.getElementById("original");
const containerTransformedImage = document.getElementById("result");
const errorLabel = document.getElementById("err");


async function transform() {
  setError();
  const file = fileInput.files[0];
  if (!file) {
    return setError("No file selected");
  }

  if (file.size > 1 * 1024 * 1024) {
    return setError("File size exceeds 1MB. Please select a smaller file.");
  }
  const arrBuf = await file.arrayBuffer();
  const imageBytes = new Uint8Array(arrBuf);
  try {
    const response = await fetch(filter.value, {
      method: "POST",
      headers: {
        "content-type": "application/octet-stream",
      },
      body: imageBytes
    });
    if (!response.ok) throw new Error("Request returned a bad response code");

    const transformedImageBlob = await response.blob();
    appendImage(containerOriginalImage, new Blob([arrBuf], { type: file.type }), "Original Image");
    appendImage(containerTransformedImage, transformedImageBlob, "Transformed Image");
  } catch (err) {
    return setError(`Error while transforming image ${err}`);
  } finally {
    return false;
  }
}

function appendImage(container, blob, caption) {
  const url = URL.createObjectURL(blob);
  container.innerHTML = '';
  const img = document.createElement("img");
  img.src = url;
  img.alt = caption;
  img.style.maxWidth = "100%";
  const header = document.createElement("h2");
  header.innerText = caption;
  header.className = "text-xl text-center"
  container.appendChild(header);
  container.appendChild(img);
}

function setError(err) {
  if (err) {
    errorLabel.innerHTML = err;
    return false;
  }
  errorLabel.innerHTML = "";
  return false;
}

