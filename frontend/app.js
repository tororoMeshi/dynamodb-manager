// Configuration
const API_BASE_URL = 'https://your-lambda-function-url.amazonaws.com'; // Replace with your actual Lambda URL

// DOM elements
const tableSelect = document.getElementById('table-select');
const refreshTablesBtn = document.getElementById('refresh-tables');
const tableInfo = document.getElementById('table-info');
const tableName = document.getElementById('table-name');
const attributesList = document.getElementById('attributes-list');
const showSampleBtn = document.getElementById('show-sample');
const sampleData = document.getElementById('sample-data');
const sampleContent = document.getElementById('sample-content');
const loading = document.getElementById('loading');
const error = document.getElementById('error');
const errorMessage = document.getElementById('error-message');
const closeErrorBtn = document.getElementById('close-error');

// State
let currentTable = null;
let currentAttributes = [];

// Utility functions
function showLoading() {
    loading.classList.remove('hidden');
}

function hideLoading() {
    loading.classList.add('hidden');
}

function showError(message) {
    errorMessage.textContent = message;
    error.classList.remove('hidden');
}

function hideError() {
    error.classList.add('hidden');
}

async function apiCall(endpoint, options = {}) {
    const url = `${API_BASE_URL}${endpoint}`;
    const response = await fetch(url, {
        headers: {
            'Content-Type': 'application/json',
            ...options.headers
        },
        ...options
    });

    if (!response.ok) {
        throw new Error(`API Error: ${response.status} ${response.statusText}`);
    }

    return await response.json();
}

// API functions
async function fetchTables() {
    try {
        showLoading();
        const tables = await apiCall('/tables');
        return tables;
    } catch (err) {
        showError(`テーブル一覧の取得に失敗しました: ${err.message}`);
        throw err;
    } finally {
        hideLoading();
    }
}

async function fetchTableInfo(tableName) {
    try {
        showLoading();
        const tableData = await apiCall(`/tables/${tableName}`);
        return tableData;
    } catch (err) {
        showError(`テーブル情報の取得に失敗しました: ${err.message}`);
        throw err;
    } finally {
        hideLoading();
    }
}

async function fetchSampleData(tableName) {
    try {
        showLoading();
        const samples = await apiCall(`/tables/${tableName}/sample`);
        return samples;
    } catch (err) {
        showError(`サンプルデータの取得に失敗しました: ${err.message}`);
        throw err;
    } finally {
        hideLoading();
    }
}

async function updateAttributeMemo(tableName, attributeName, description) {
    try {
        showLoading();
        await apiCall(`/tables/${tableName}/${attributeName}`, {
            method: 'PUT',
            body: JSON.stringify({ description })
        });
    } catch (err) {
        showError(`メモの更新に失敗しました: ${err.message}`);
        throw err;
    } finally {
        hideLoading();
    }
}

// UI functions
function populateTableSelect(tables) {
    tableSelect.innerHTML = '<option value="">テーブルを選択してください</option>';
    tables.forEach(table => {
        const option = document.createElement('option');
        option.value = table;
        option.textContent = table;
        tableSelect.appendChild(option);
    });
}

function createAttributeItem(attribute) {
    const item = document.createElement('div');
    item.className = 'attribute-item';
    
    item.innerHTML = `
        <div class="attribute-header">
            <span class="attribute-name">${attribute.attribute_name}</span>
            <span class="attribute-type ${attribute.type_hint}">${attribute.type_hint}</span>
        </div>
        <div class="memo-section">
            <textarea placeholder="この属性について説明やメモを記入してください..." 
                      id="memo-${attribute.attribute_name}">${attribute.description || ''}</textarea>
            <button onclick="saveMemo('${attribute.attribute_name}')">保存</button>
        </div>
    `;
    
    return item;
}

function displayTableInfo(tableData) {
    currentTable = tableData.table_name;
    currentAttributes = tableData.attributes;
    
    tableName.textContent = tableData.table_name;
    
    attributesList.innerHTML = '';
    if (tableData.attributes.length === 0) {
        attributesList.innerHTML = '<p>属性情報がありません。</p>';
    } else {
        tableData.attributes.forEach(attribute => {
            const item = createAttributeItem(attribute);
            attributesList.appendChild(item);
        });
    }
    
    tableInfo.classList.remove('hidden');
    sampleData.classList.add('hidden');
}

function displaySampleData(samples) {
    sampleContent.innerHTML = '';
    
    if (samples.length === 0) {
        sampleContent.innerHTML = '<p>サンプルデータがありません。</p>';
    } else {
        samples.forEach((sample, index) => {
            const item = document.createElement('div');
            item.className = 'sample-item';
            item.innerHTML = `
                <h4>サンプル ${index + 1}</h4>
                <pre>${JSON.stringify(sample, null, 2)}</pre>
            `;
            sampleContent.appendChild(item);
        });
    }
    
    sampleData.classList.remove('hidden');
}

// Event handlers
async function handleRefreshTables() {
    try {
        const tables = await fetchTables();
        populateTableSelect(tables);
    } catch (err) {
        console.error('Failed to refresh tables:', err);
    }
}

async function handleTableSelect() {
    const selectedTable = tableSelect.value;
    if (!selectedTable) {
        tableInfo.classList.add('hidden');
        return;
    }
    
    try {
        const tableData = await fetchTableInfo(selectedTable);
        displayTableInfo(tableData);
    } catch (err) {
        console.error('Failed to fetch table info:', err);
    }
}

async function handleShowSample() {
    if (!currentTable) return;
    
    try {
        const samples = await fetchSampleData(currentTable);
        displaySampleData(samples);
    } catch (err) {
        console.error('Failed to fetch sample data:', err);
    }
}

async function saveMemo(attributeName) {
    if (!currentTable) return;
    
    const textarea = document.getElementById(`memo-${attributeName}`);
    const description = textarea.value.trim();
    
    try {
        await updateAttributeMemo(currentTable, attributeName, description);
        
        // Update local state
        const attribute = currentAttributes.find(attr => attr.attribute_name === attributeName);
        if (attribute) {
            attribute.description = description;
        }
        
        // Show success feedback
        const button = textarea.nextElementSibling;
        const originalText = button.textContent;
        button.textContent = '保存完了!';
        button.style.background = '#27ae60';
        
        setTimeout(() => {
            button.textContent = originalText;
            button.style.background = '';
        }, 2000);
        
    } catch (err) {
        console.error('Failed to save memo:', err);
    }
}

// Initialize
document.addEventListener('DOMContentLoaded', function() {
    refreshTablesBtn.addEventListener('click', handleRefreshTables);
    tableSelect.addEventListener('change', handleTableSelect);
    showSampleBtn.addEventListener('click', handleShowSample);
    closeErrorBtn.addEventListener('click', hideError);
    
    // Load tables on startup
    handleRefreshTables();
});

// Make saveMemo globally accessible
window.saveMemo = saveMemo;