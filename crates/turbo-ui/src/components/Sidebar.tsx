import React from 'react';
import { Layout, Menu } from 'antd';
import { useNavigate, useLocation } from 'react-router-dom';
import { 
  HomeOutlined, 
  DownloadOutlined, 
  RadarChartOutlined, 
  SettingOutlined 
} from '@ant-design/icons';
import './Sidebar.css';

const { Sider } = Layout;

const Sidebar: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();

  const menuItems = [
    {
      key: '/',
      icon: <HomeOutlined />,
      label: '首页',
    },
    {
      key: '/download',
      icon: <DownloadOutlined />,
      label: '下载',
    },
    {
      key: '/radar',
      icon: <RadarChartOutlined />,
      label: '雷达',
    },
    {
      key: '/settings',
      icon: <SettingOutlined />,
      label: '设置',
    },
  ];

  const handleMenuClick = (key: string) => {
    navigate(key);
  };

  return (
    <Sider 
      width={220} 
      className="app-sidebar"
      breakpoint="lg"
      collapsedWidth={0}
    >
      <div className="logo">
        <DownloadOutlined style={{ fontSize: 24, color: '#1890ff' }} />
        <span className="logo-text">Turbo Download</span>
      </div>
      <Menu
        mode="inline"
        selectedKeys={[location.pathname]}
        items={menuItems}
        onClick={({ key }) => handleMenuClick(key)}
        className="sidebar-menu"
      />
    </Sider>
  );
};

export default Sidebar;