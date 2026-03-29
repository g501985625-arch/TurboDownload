import React from 'react';
import { Layout, Typography } from 'antd';
import { useLocation } from 'react-router-dom';
import './Header.css';

const { Header: AntHeader } = Layout;
const { Title } = Typography;

const pageTitles: Record<string, string> = {
  '/': '首页',
  '/download': '下载管理',
  '/radar': '资源雷达',
  '/settings': '设置',
};

const Header: React.FC = () => {
  const location = useLocation();
  const title = pageTitles[location.pathname] || 'Turbo Download';

  return (
    <AntHeader className="app-header">
      <Title level={3} className="page-title">{title}</Title>
    </AntHeader>
  );
};

export default Header;