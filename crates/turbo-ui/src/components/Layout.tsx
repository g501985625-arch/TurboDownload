import React from 'react';
import { Layout as AntLayout } from 'antd';
import Sidebar from './Sidebar';
import Header from './Header';
import './Layout.css';

const { Content } = AntLayout;

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  return (
    <AntLayout className="app-layout">
      <Sidebar />
      <AntLayout className="main-layout">
        <Header />
        <Content className="main-content">
          {children}
        </Content>
      </AntLayout>
    </AntLayout>
  );
};

export default Layout;