const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

document.addEventListener('DOMContentLoaded', async () => {
  const dropZone = document.getElementById('drop-zone');
  const statusContainer = document.getElementById('status-container');
  const resultContainer = document.getElementById('result-container');
  const statusMsg = document.getElementById('status-msg');
  const resultMsg = document.getElementById('result-msg');
  const resetBtn = document.getElementById('reset-btn');

  // Tauriのドラッグ＆ドロップイベントをリッスン
  await listen('tauri://drag-drop', (event) => {
    const paths = event.payload.paths;
    if (paths && paths.length > 0) {
      const path = paths[0];
      if (path.endsWith('.txt')) {
        processFile(path);
      } else {
        alert('TXTファイルのみ対応しています。');
      }
    }
  });

  // UI上の視覚効果（通常のブラウザイベントも併用）
  dropZone.addEventListener('dragover', (e) => {
    e.preventDefault();
    dropZone.classList.add('drag-over');
  });

  dropZone.addEventListener('dragleave', () => {
    dropZone.classList.remove('drag-over');
  });

  // クリックでファイル選択（オプション）
  dropZone.addEventListener('click', async () => {
    // Tauri 2のdialogプラグインを使う場合は追加設定が必要なので
    // 今回はドラッグ＆ドロップをメインとする
  });

  resetBtn.addEventListener('click', () => {
    resultContainer.classList.add('hidden');
    dropZone.classList.remove('hidden');
  });

  async function processFile(path) {
    dropZone.classList.add('hidden');
    statusContainer.classList.remove('hidden');
    statusMsg.textContent = `${path} を処理中...`;

    try {
      const result = await invoke('process_file', { txtPath: path });
      statusContainer.classList.add('hidden');
      resultContainer.classList.remove('hidden');
      resultMsg.textContent = result;
    } catch (error) {
      statusContainer.classList.add('hidden');
      dropZone.classList.remove('hidden');
      alert(`エラーが発生しました: ${error}`);
    }
  }
});
