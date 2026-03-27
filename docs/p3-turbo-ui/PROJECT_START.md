# P3: turbo-ui 项目启动文档

## 项目概述

**项目名称**: turbo-ui (前端 UI 框架)  
**项目类型**: TypeScript/React 库  
**预估工时**: 45 人时  
**优先级**: P1 (高) - 前端核心  
**依赖项**: 无 (独立前端库)

### 核心功能

- 下载任务列表组件
- 任务详情面板
- 进度可视化组件
- 设置面板
- 主题系统 (亮/暗模式)
- 国际化支持
- 响应式布局
- 状态管理 (Zustand)

---

## 技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| React | 18.x | UI 框架 |
| TypeScript | 5.x | 类型系统 |
| Vite | 5.x | 构建工具 |
| Tailwind CSS | 3.x | 样式框架 |
| Zustand | 4.x | 状态管理 |
| React Router | 6.x | 路由管理 |
| React Query | 5.x | 数据获取 |
| Lucide React | 0.x | 图标库 |
| clsx | 2.x | 类名工具 |
| tailwind-merge | 2.x | Tailwind 合并 |

---

## 项目初始化步骤

### 步骤 1: 创建项目目录

```bash
# 进入项目根目录
cd ~/Projects/TurboDownload

# 使用 Vite 创建 React TypeScript 项目
npm create vite@latest packages/turbo-ui -- --template react-ts

# 进入项目目录
cd packages/turbo-ui
```

**预期输出**:
```
Scaffolding a new project in /Users/.../turbo-ui...
Done. Now run:
  cd packages/turbo-ui
  npm install
```

### 步骤 2: 安装核心依赖

```bash
# 安装核心依赖
npm install zustand @tanstack/react-query react-router-dom

# 安装 UI 依赖
npm install lucide-react clsx tailwind-merge

# 安装 Tailwind CSS
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# 安装开发工具
npm install -D @types/node eslint @typescript-eslint/eslint-plugin @typescript-eslint/parser
npm install -D eslint-plugin-react eslint-plugin-react-hooks eslint-plugin-jsx-a11y
npm install -D prettier eslint-config-prettier eslint-plugin-prettier
npm install -D vitest @testing-library/react @testing-library/jest-dom jsdom
```

### 步骤 3: 创建目录结构

```bash
# 创建源代码目录结构
mkdir -p src/{components,hooks,stores,types,utils,styles,locales}
mkdir -p src/components/{common,tasks,settings,layout}
mkdir -p src/components/tasks/{list,detail,progress}
mkdir -p src/stores/{task,settings,ui}

# 创建测试目录
mkdir -p src/__tests__

# 创建资源目录
mkdir -p public/icons
```

**最终目录结构**:
```
packages/turbo-ui/
├── index.html
├── package.json
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
├── .eslintrc.cjs
├── .prettierrc
├── public/
│   └── icons/
├── src/
│   ├── main.tsx              # 应用入口
│   ├── App.tsx               # 根组件
│   ├── vite-env.d.ts         # Vite 类型声明
│   ├── components/
│   │   ├── common/
│   │   │   ├── Button/
│   │   │   │   ├── Button.tsx
│   │   │   │   ├── Button.test.tsx
│   │   │   │   └── index.ts
│   │   │   ├── Input/
│   │   │   ├── Modal/
│   │   │   ├── Progress/
│   │   │   ├── Toast/
│   │   │   └── Tooltip/
│   │   ├── layout/
│   │   │   ├── Header/
│   │   │   ├── Sidebar/
│   │   │   ├── Footer/
│   │   │   └── MainLayout/
│   │   ├── tasks/
│   │   │   ├── list/
│   │   │   │   ├── TaskList.tsx
│   │   │   │   ├── TaskItem.tsx
│   │   │   │   └── TaskFilters.tsx
│   │   │   ├── detail/
│   │   │   │   ├── TaskDetail.tsx
│   │   │   │   ├── TaskInfo.tsx
│   │   │   │   └── TaskActions.tsx
│   │   │   └── progress/
│   │   │       ├── ProgressBar.tsx
│   │   │       ├── SpeedIndicator.tsx
│   │   │       └── ETAIndicator.tsx
│   │   └── settings/
│   │       ├── SettingsPanel.tsx
│   │       ├── GeneralSettings.tsx
│   │       ├── DownloadSettings.tsx
│   │       └── AppearanceSettings.tsx
│   ├── hooks/
│   │   ├── useTasks.ts
│   │   ├── useSettings.ts
│   │   ├── useTheme.ts
│   │   └── useToast.ts
│   ├── stores/
│   │   ├── task/
│   │   │   ├── taskStore.ts
│   │   │   └── taskSelectors.ts
│   │   ├── settings/
│   │   │   └── settingsStore.ts
│   │   └── ui/
│   │       └── uiStore.ts
│   ├── types/
│   │   ├── task.ts
│   │   ├── settings.ts
│   │   ├── events.ts
│   │   └── api.ts
│   ├── utils/
│   │   ├── format.ts
│   │   ├── cn.ts
│   │   ├── storage.ts
│   │   └── validators.ts
│   ├── styles/
│   │   ├── globals.css
│   │   └── themes.css
│   └── locales/
│       ├── en.json
│       └── zh-CN.json
└── src/__tests__/
    ├── setup.ts
    └── utils.tsx
```

### 步骤 4: 配置 Tailwind CSS

创建 `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f0f9ff',
          100: '#e0f2fe',
          200: '#bae6fd',
          300: '#7dd3fc',
          400: '#38bdf8',
          500: '#0ea5e9',
          600: '#0284c7',
          700: '#0369a1',
          800: '#075985',
          900: '#0c4a6e',
        },
        success: {
          50: '#f0fdf4',
          500: '#22c55e',
          600: '#16a34a',
        },
        warning: {
          50: '#fffbeb',
          500: '#f59e0b',
          600: '#d97706',
        },
        danger: {
          50: '#fef2f2',
          500: '#ef4444',
          600: '#dc2626',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      animation: {
        'spin-slow': 'spin 3s linear infinite',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'progress': 'progress 1s ease-in-out infinite',
      },
      keyframes: {
        progress: {
          '0%': { width: '0%' },
          '50%': { width: '70%' },
          '100%': { width: '100%' },
        },
      },
    },
  },
  plugins: [],
}
```

创建 `postcss.config.js`:

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

### 步骤 5: 配置 TypeScript

更新 `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"],
      "@components/*": ["./src/components/*"],
      "@hooks/*": ["./src/hooks/*"],
      "@stores/*": ["./src/stores/*"],
      "@types/*": ["./src/types/*"],
      "@utils/*": ["./src/utils/*"],
      "@styles/*": ["./src/styles/*"]
    }
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

### 步骤 6: 配置 Vite

更新 `vite.config.ts`:

```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@components': path.resolve(__dirname, './src/components'),
      '@hooks': path.resolve(__dirname, './src/hooks'),
      '@stores': path.resolve(__dirname, './src/stores'),
      '@types': path.resolve(__dirname, './src/types'),
      '@utils': path.resolve(__dirname, './src/utils'),
      '@styles': path.resolve(__dirname, './src/styles'),
    },
  },
  server: {
    port: 3000,
    host: true,
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
    lib: {
      entry: path.resolve(__dirname, 'src/index.ts'),
      name: 'TurboUI',
      formats: ['es', 'cjs'],
      fileName: (format) => `turbo-ui.${format}.js`,
    },
    rollupOptions: {
      external: ['react', 'react-dom'],
      output: {
        globals: {
          react: 'React',
          'react-dom': 'ReactDOM',
        },
      },
    },
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/__tests__/setup.ts',
  },
})
```

---

## 开发环境配置

### 1. 配置 ESLint

创建 `.eslintrc.cjs`:

```javascript
module.exports = {
  root: true,
  env: { browser: true, es2020: true, node: true },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:react/recommended',
    'plugin:react/jsx-runtime',
    'plugin:react-hooks/recommended',
    'plugin:jsx-a11y/recommended',
    'prettier',
  ],
  ignorePatterns: ['dist', '.eslintrc.cjs'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true,
    },
  },
  plugins: [
    'react',
    '@typescript-eslint',
    'react-hooks',
    'jsx-a11y',
    'prettier',
  ],
  settings: {
    react: {
      version: 'detect',
    },
  },
  rules: {
    'react/react-in-jsx-scope': 'off',
    'react/prop-types': 'off',
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    '@typescript-eslint/explicit-function-return-type': 'off',
    '@typescript-eslint/explicit-module-boundary-types': 'off',
    'prettier/prettier': 'error',
  },
}
```

### 2. 配置 Prettier

创建 `.prettierrc`:

```json
{
  "semi": false,
  "singleQuote": true,
  "trailingComma": "es5",
  "tabWidth": 2,
  "printWidth": 100,
  "bracketSpacing": true,
  "jsxSingleQuote": false,
  "arrowParens": "always",
  "endOfLine": "lf"
}
```

### 3. 配置测试环境

创建 `src/__tests__/setup.ts`:

```typescript
import '@testing-library/jest-dom'
import { cleanup } from '@testing-library/react'
import { afterEach, vi } from 'vitest'

afterEach(() => {
  cleanup()
})

// Mock window.matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
})

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))
```

---

## 依赖安装命令

```bash
# 安装所有依赖
cd ~/Projects/TurboDownload/packages/turbo-ui
npm install

# 验证安装
npm run dev
```

---

## 快速开始

### 创建全局样式

创建 `src/styles/globals.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 222.2 84% 4.9%;
    --card: 0 0% 100%;
    --card-foreground: 222.2 84% 4.9%;
    --primary: 199 89% 48%;
    --primary-foreground: 210 40% 98%;
    --secondary: 210 40% 96%;
    --secondary-foreground: 222.2 47.4% 11.2%;
    --muted: 210 40% 96%;
    --muted-foreground: 215.4 16.3% 46.9%;
    --accent: 210 40% 96%;
    --accent-foreground: 222.2 47.4% 11.2%;
    --destructive: 0 84.2% 60.2%;
    --destructive-foreground: 210 40% 98%;
    --border: 214.3 31.8% 91.4%;
    --input: 214.3 31.8% 91.4%;
    --ring: 199 89% 48%;
    --radius: 0.5rem;
  }

  .dark {
    --background: 222.2 84% 4.9%;
    --foreground: 210 40% 98%;
    --card: 222.2 84% 4.9%;
    --card-foreground: 210 40% 98%;
    --primary: 199 89% 48%;
    --primary-foreground: 222.2 47.4% 11.2%;
    --secondary: 217.2 32.6% 17.5%;
    --secondary-foreground: 210 40% 98%;
    --muted: 217.2 32.6% 17.5%;
    --muted-foreground: 215 20.2% 65.1%;
    --accent: 217.2 32.6% 17.5%;
    --accent-foreground: 210 40% 98%;
    --destructive: 0 62.8% 30.6%;
    --destructive-foreground: 210 40% 98%;
    --border: 217.2 32.6% 17.5%;
    --input: 217.2 32.6% 17.5%;
    --ring: 199 89% 48%;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground;
    font-feature-settings: "rlig" 1, "calt" 1;
  }
}
```

### 创建第一个组件

创建 `src/components/common/Button/Button.tsx`:

```tsx
import { forwardRef, ButtonHTMLAttributes } from 'react'
import { cn } from '@utils/cn'

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  isLoading?: boolean
  leftIcon?: React.ReactNode
  rightIcon?: React.ReactNode
}

const variantStyles = {
  primary: 'bg-primary-500 text-white hover:bg-primary-600 focus:ring-primary-500',
  secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
  outline: 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
  ghost: 'hover:bg-accent hover:text-accent-foreground',
  danger: 'bg-danger-500 text-white hover:bg-danger-600 focus:ring-danger-500',
}

const sizeStyles = {
  sm: 'h-8 px-3 text-xs',
  md: 'h-10 px-4 text-sm',
  lg: 'h-12 px-6 text-base',
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      className,
      variant = 'primary',
      size = 'md',
      isLoading = false,
      leftIcon,
      rightIcon,
      disabled,
      children,
      ...props
    },
    ref
  ) => {
    return (
      <button
        ref={ref}
        className={cn(
          'inline-flex items-center justify-center gap-2 rounded-md font-medium',
          'transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2',
          'disabled:pointer-events-none disabled:opacity-50',
          variantStyles[variant],
          sizeStyles[size],
          className
        )}
        disabled={disabled || isLoading}
        {...props}
      >
        {isLoading ? (
          <svg
            className="h-4 w-4 animate-spin"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
        ) : (
          leftIcon
        )}
        {children}
        {rightIcon}
      </button>
    )
  }
)

Button.displayName = 'Button'
```

---

## 开发工作流

### 日常开发

```bash
# 启动开发服务器
npm run dev

# 运行测试
npm run test

# 运行测试 (监视模式)
npm run test:watch

# 生成测试覆盖率
npm run test:coverage

# 类型检查
npm run typecheck

# 代码检查
npm run lint

# 格式化代码
npm run format
```

### 构建发布

```bash
# 构建库
npm run build

# 构建故事书
npm run storybook:build

# 预览构建结果
npm run preview
```

---

## 环境验证清单

- [ ] Node.js 18+ 安装完成 (`node --version`)
- [ ] npm 可用 (`npm --version`)
- [ ] 项目依赖安装成功 (`npm install`)
- [ ] 开发服务器启动成功 (`npm run dev`)
- [ ] 测试通过 (`npm run test`)
- [ ] ESLint 无错误 (`npm run lint`)
- [ ] TypeScript 编译通过 (`npm run typecheck`)

---

## 故障排除

### 常见问题

#### 1. Tailwind 样式不生效

确保导入了全局样式:
```tsx
// main.tsx
import '@styles/globals.css'
```

#### 2. TypeScript 路径别名不工作

确保 `tsconfig.json` 和 `vite.config.ts` 中的路径别名一致。

#### 3. 热更新不工作

检查 Vite 配置中的 server 设置:
```typescript
server: {
  port: 3000,
  host: true,
  hmr: true,
}
```

---

## 相关文档

- [TASK_CHAIN.md](./TASK_CHAIN.md) - 详细任务链
- [DEV_CHECKLIST.md](./DEV_CHECKLIST.md) - 开发步骤清单
- [CODE_TEMPLATES.md](./CODE_TEMPLATES.md) - 代码模板
- [../API_CONTRACTS.md](../API_CONTRACTS.md) - 接口契约