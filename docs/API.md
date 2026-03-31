# TurboDownload Agent API

## 基础信息

- **基础 URL**: `http://localhost:8080/api/v1`
- **WebSocket**: `ws://localhost:8080/ws`
- **协议**: HTTP/1.1, WebSocket

## 认证

- **类型**: Bearer Token
- **Header**: `Authorization: Bearer <token>`
- **说明**: 大部分端点需要认证，部分端点（如健康检查）无需认证

## 错误响应格式

所有错误响应均遵循以下格式：

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "错误描述信息"
  }
}
```

常见错误码：
- `INVALID_URL` - URL 格式无效
- `DOWNLOAD_NOT_FOUND` - 下载任务不存在
- `INVALID_STATE` - 当前状态无法执行此操作
- `AUTH_REQUIRED` - 需要认证
- `INTERNAL_ERROR` - 服务器内部错误

---

## 端点

### 健康检查

检查服务是否正常运行。

```
GET /health
```

**响应 (200 OK)**:
```json
{
  "status": "ok",
  "version": "1.0.0"
}
```

---

### 创建下载

创建一个新的下载任务。

```
POST /api/v1/download
Content-Type: application/json
Authorization: Bearer <token>
```

**请求体**:
```json
{
  "url": "https://example.com/file.zip",
  "filename": "file.zip",
  "threads": 4,
  "options": {
    "timeout": 300,
    "retry": 3,
    "proxy": ""
  }
}
```

**字段说明**:
| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| url | string | 是 | 下载链接 |
| filename | string | 否 | 保存文件名（默认从URL提取） |
| threads | number | 否 | 并行线程数（默认4，最大16） |
| options.timeout | number | 否 | 超时时间（秒） |
| options.retry | number | 否 | 重试次数 |
| options.proxy | string | 否 | 代理地址 |

**响应 (200 OK)**:
```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "pending",
  "url": "https://example.com/file.zip",
  "filename": "file.zip",
  "threads": 4,
  "created_at": "2026-03-30T12:00:00Z"
}
```

---

### 获取下载状态

获取指定下载任务的详细信息和进度。

```
GET /api/v1/download/:id
Authorization: Bearer <token>
```

**路径参数**:
- `id`: 任务ID (UUID)

**响应 (200 OK)**:
```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "downloading",
  "url": "https://example.com/file.zip",
  "filename": "file.zip",
  "threads": 4,
  "progress": {
    "downloaded": 1024000,
    "total": 2048000,
    "speed": 102400,
    "percent": 50
  },
  "created_at": "2026-03-30T12:00:00Z",
  "started_at": "2026-03-30T12:00:05Z"
}
```

**状态说明**:
| 状态 | 说明 |
|------|------|
| pending | 等待中 |
| downloading | 下载中 |
| paused | 已暂停 |
| completed | 已完成 |
| failed | 失败 |
| cancelled | 已取消 |

---

### 暂停下载

暂停一个正在下载的任务。

```
POST /api/v1/download/:id/pause
Authorization: Bearer <token>
```

**路径参数**:
- `id`: 任务ID (UUID)

**响应 (200 OK)**:
```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "paused"
}
```

---

### 恢复下载

恢复一个已暂停的下载任务。

```
POST /api/v1/download/:id/resume
Authorization: Bearer <token>
```

**路径参数**:
- `id`: 任务ID (UUID)

**响应 (200 OK)**:
```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "downloading"
}
```

---

### 取消下载

取消一个下载任务。

```
DELETE /api/v1/download/:id
Authorization: Bearer <token>
```

**路径参数**:
- `id`: 任务ID (UUID)

**响应 (200 OK)**:
```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "cancelled"
}
```

---

### 列出任务

获取当前用户的所有下载任务列表。

```
GET /api/v1/downloads
Authorization: Bearer <token>
```

**查询参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| status | string | 按状态过滤 (pending/downloading/paused/completed/failed/cancelled) |
| limit | number | 返回数量限制 (默认50) |
| offset | number | 分页偏移量 |

**响应 (200 OK)**:
```json
{
  "tasks": [
    {
      "task_id": "550e8400-e29b-41d4-a716-446655440000",
      "status": "downloading",
      "url": "https://example.com/file.zip",
      "filename": "file.zip",
      "progress": {
        "downloaded": 1024000,
        "total": 2048000,
        "percent": 50
      },
      "created_at": "2026-03-30T12:00:00Z"
    }
  ],
  "total": 1,
  "limit": 50,
  "offset": 0
}
```

---

## WebSocket 事件

连接到 WebSocket 端点 `ws://localhost:8080/ws` 可实时接收下载进度和状态更新。

**认证**: WebSocket 连接同样需要 Bearer Token 认证：
```
ws://localhost:8080/ws?token=<token>
```

### Progress

下载进度更新事件。

```json
{
  "type": "Progress",
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "downloaded": 1024000,
  "total": 2048000,
  "speed": 102400,
  "percent": 50
}
```

**字段说明**:
| 字段 | 类型 | 说明 |
|------|------|------|
| type | string | 事件类型 ("Progress") |
| task_id | string | 任务ID |
| downloaded | number | 已下载字节数 |
| total | number | 文件总字节数 |
| speed | number | 当前下载速度 (bytes/s) |
| percent | number | 下载百分比 (0-100) |

---

### Completed

下载完成事件。

```json
{
  "type": "Completed",
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "filename": "file.zip",
  "path": "/downloads/file.zip",
  "size": 2048000,
  "duration": 20.5
}
```

**字段说明**:
| 字段 | 类型 | 说明 |
|------|------|------|
| type | string | 事件类型 ("Completed") |
| task_id | string | 任务ID |
| filename | string | 文件名 |
| path | string | 保存路径 |
| size | number | 文件大小 (bytes) |
| duration | number | 下载耗时 (秒) |

---

### Failed

下载失败事件。

```json
{
  "type": "Failed",
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "error": "Connection timeout",
  "code": "TIMEOUT"
}
```

---

### StatusChanged

任务状态变更事件。

```json
{
  "type": "StatusChanged",
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "old_status": "pending",
  "new_status": "downloading"
}
```

---

## 使用示例

### cURL

```bash
# 健康检查
curl -s http://localhost:8080/health

# 创建下载
curl -s -X POST http://localhost:8080/api/v1/download \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_token_here" \
  -d '{"url":"https://example.com/file.zip","filename":"file.zip","threads":4}'

# 获取状态
curl -s http://localhost:8080/api/v1/download/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer your_token_here"

# 列出任务
curl -s "http://localhost:8080/api/v1/downloads?status=downloading" \
  -H "Authorization: Bearer your_token_here"

# 暂停下载
curl -s -X POST http://localhost:8080/api/v1/download/550e8400-e29b-41d4-a716-446655440000/pause \
  -H "Authorization: Bearer your_token_here"

# 恢复下载
curl -s -X POST http://localhost:8080/api/v1/download/550e8400-e29b-41d4-a716-446655440000/resume \
  -H "Authorization: Bearer your_token_here"

# 取消下载
curl -s -X DELETE http://localhost:8080/api/v1/download/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer your_token_here"
```

### JavaScript

```javascript
const BASE_URL = 'http://localhost:8080/api/v1';
const WS_URL = 'ws://localhost:8080/ws';
const TOKEN = 'your_token_here';

const headers = {
  'Content-Type': 'application/json',
  'Authorization': `Bearer ${TOKEN}`
};

// 创建下载
async function createDownload(url, filename, threads = 4) {
  const response = await fetch(`${BASE_URL}/download`, {
    method: 'POST',
    headers,
    body: JSON.stringify({ url, filename, threads })
  });
  return response.json();
}

// 获取下载状态
async function getDownloadStatus(taskId) {
  const response = await fetch(`${BASE_URL}/download/${taskId}`, { headers });
  return response.json();
}

// 列出所有下载
async function listDownloads(status) {
  const params = status ? `?status=${status}` : '';
  const response = await fetch(`${BASE_URL}/downloads${params}`, { headers });
  return response.json();
}

// 暂停下载
async function pauseDownload(taskId) {
  const response = await fetch(`${BASE_URL}/download/${taskId}/pause`, {
    method: 'POST',
    headers
  });
  return response.json();
}

// 恢复下载
async function resumeDownload(taskId) {
  const response = await fetch(`${BASE_URL}/download/${taskId}/resume`, {
    method: 'POST',
    headers
  });
  return response.json();
}

// 取消下载
async function cancelDownload(taskId) {
  const response = await fetch(`${BASE_URL}/download/${taskId}`, {
    method: 'DELETE',
    headers
  });
  return response.json();
}

// WebSocket 监听
function connectWebSocket() {
  const ws = new WebSocket(`${WS_URL}?token=${TOKEN}`);
  
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    
    switch (data.type) {
      case 'Progress':
        console.log(`下载进度: ${data.percent}% (${data.speed} bytes/s)`);
        break;
      case 'Completed':
        console.log(`下载完成: ${data.filename}`);
        break;
      case 'Failed':
        console.log(`下载失败: ${data.error}`);
        break;
      case 'StatusChanged':
        console.log(`状态变更: ${data.old_status} -> ${data.new_status}`);
        break;
    }
  };
  
  return ws;
}
```

---

## 速率限制

- **API 请求**: 100 请求/分钟
- **WebSocket 连接**: 5 连接/用户

---

## 版本历史

| 版本 | 日期 | 说明 |
|------|------|------|
| 1.0.0 | 2026-03-30 | 初始版本 |