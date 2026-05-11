const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

let selectedFiles = [];

const dropZone = document.getElementById("drop-zone");
const selectBtn = document.getElementById("select-btn");
const convertBtn = document.getElementById("convert-btn");
const fileList = document.getElementById("file-list");
const statusText = document.getElementById("status-text");

function updateFileList() {
  fileList.innerHTML = "";
  selectedFiles.forEach((file, index) => {
    const item = document.createElement("div");
    item.className = "file-item";
    item.innerHTML = `
      <span><i class="fa-regular fa-file-lines" style="margin-right: 8px; color: var(--accent-color);"></i>${file.name}</span>
      <span class="remove-btn" data-index="${index}"><i class="fa-solid fa-xmark"></i></span>
    `;
    fileList.appendChild(item);
  });
  
  convertBtn.disabled = selectedFiles.length === 0;
  statusText.textContent = selectedFiles.length > 0 ? `${selectedFiles.length} file(s) selected` : "Ready to convert";
}

selectBtn.addEventListener("click", async (e) => {
  e.stopPropagation();
  try {
    const selected = await open({
      multiple: true,
      filters: [{ name: "Text", extensions: ["txt"] }]
    });

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      paths.forEach(path => {
        const name = path.split(/[/\\]/).pop();
        if (!selectedFiles.some(f => f.path === path)) {
          selectedFiles.push({ name, path });
        }
      });
      updateFileList();
    }
  } catch (err) {
    console.error("Failed to open dialog:", err);
  }
});

// Drag & Drop
dropZone.addEventListener("dragover", (e) => {
  e.preventDefault();
  dropZone.classList.add("drag-over");
});

dropZone.addEventListener("dragleave", () => {
  dropZone.classList.remove("drag-over");
});

dropZone.addEventListener("drop", async (e) => {
  e.preventDefault();
  dropZone.classList.remove("drag-over");
  
  // Note: Standard web drag & drop doesn't give full paths for security.
  // Tauri has a special 'tauri://drag-drop' event for this.
  // For now we rely on the Select button or use the Tauri global listener.
});

fileList.addEventListener("click", (e) => {
  const removeBtn = e.target.closest(".remove-btn");
  if (removeBtn) {
    const index = parseInt(removeBtn.dataset.index);
    selectedFiles.splice(index, 1);
    updateFileList();
  }
});

convertBtn.addEventListener("click", async () => {
  statusText.textContent = "Converting...";
  statusText.style.color = "var(--accent-color)";
  convertBtn.disabled = true;
  
  const settings = {
    orientation: document.querySelector('input[name="orientation"]:checked').value,
    paper_orientation: document.querySelector('input[name="paper-orientation"]:checked').value,
    nombre: document.getElementById("nombre-toggle").checked,
    nombre_position: document.querySelector('input[name="nombre-position"]:checked').value,
    chars_per_line: parseInt(document.getElementById("chars-per-line").value),
    lines_per_page: parseInt(document.getElementById("lines-per-page").value),
  };

  try {
    for (const file of selectedFiles) {
      await invoke("convert_text_to_docx", { 
        filePath: file.path,
        settings 
      });
    }
    statusText.textContent = "Success! Files converted.";
    statusText.style.color = "#4ade80";
    selectedFiles = [];
    updateFileList();
  } catch (error) {
    console.error(error);
    statusText.textContent = "Error: " + error;
    statusText.style.color = "#f87171";
  } finally {
    convertBtn.disabled = selectedFiles.length === 0;
  }
});

// Tauri 2 Drag & Drop Listener
if (window.__TAURI__) {
  const { listen } = window.__TAURI__.event;
  listen("tauri://drag-drop", (event) => {
    const paths = event.payload.paths;
    if (paths) {
      paths.forEach(path => {
        if (path.endsWith(".txt")) {
          const name = path.split(/[/\\]/).pop();
          if (!selectedFiles.some(f => f.path === path)) {
            selectedFiles.push({ name, path });
          }
        }
      });
      updateFileList();
    }
  });
}
