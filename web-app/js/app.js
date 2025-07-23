// Note: With --target no-modules, we don't use ES6 imports
// The WASM module will be available globally as wasm_bindgen

let wasmModule = null;

// Initialize WASM module
async function initWasm() {
    try {
        if (typeof wasm_bindgen === 'undefined') {
            throw new Error('wasm_bindgen not available. Make sure wallet_core.js is loaded.');
        }
        // Modern initialization: pass an object with 'url' property
        wasmModule = await wasm_bindgen({ url: './js/pkg/wallet_core_bg.wasm' });
        updateStatus('DAPA Wallet Generator ready! 🚀', 'success');
    } catch (error) {
        updateStatus('Failed to load wallet generator. Please refresh the page.', 'error');
        console.error(error);
    }
}

function updateStatus(message, type = 'loading') {
    const statusDiv = document.getElementById('status');
    statusDiv.textContent = message;
    statusDiv.className = `status ${type}`;
    statusDiv.style.display = 'block';
    if (type === 'success') {
        setTimeout(() => {
            statusDiv.style.display = 'none';
        }, 3000);
    }
}

// Generate QR code
function generateQRCode(elementId, text) {
    try {
        const qr = qrcode(0, 'M');
        qr.addData(text);
        qr.make();
        const element = document.getElementById(elementId);
        element.innerHTML = qr.createImgTag(4, 8);
    } catch (error) {
        document.getElementById(elementId).innerHTML = '<p>QR generation failed</p>';
    }
}

// Helper to safely set textContent
function setText(id, value) {
    const el = document.getElementById(id);
    if (el) el.textContent = value;
    else console.warn(`Element with id '${id}' not found.`);
}

// Generate new wallet
async function generateNewWallet() {
    if (!wasmModule) {
        updateStatus('Please wait for the wallet generator to load...', 'error');
        return;
    }
    const generateBtn = document.getElementById('generateBtn');
    generateBtn.disabled = true;
    generateBtn.textContent = 'Generating...';
    try {
        updateStatus('Generating secure random wallet...', 'loading');
        const wallet = wasm_bindgen.generate_wallet();
        setText('walletSeed', wallet.seed);
        setText('walletAddress', wallet.address);
        setText('privateKey', wallet.private_key);
        setText('publicKey', wallet.public_key);
        generateQRCode('addressQR', wallet.address);
        generateQRCode('privateKeyQR', wallet.private_key);
        document.getElementById('walletInfo').style.display = 'block';
        updateStatus('Wallet generated successfully! 🎉', 'success');
    } catch (error) {
        updateStatus('Failed to generate wallet.', 'error');
        console.error(error);
    } finally {
        generateBtn.disabled = false;
        generateBtn.textContent = 'Generate Wallet';
    }
}

window.addEventListener('DOMContentLoaded', () => {
    document.getElementById('generateBtn').onclick = generateNewWallet;
    document.getElementById('walletInfo').style.display = 'none';
    initWasm();
});