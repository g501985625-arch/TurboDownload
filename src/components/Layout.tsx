import React from 'react';
import { Layout as AntLayout } from 'antd';
import Sidebar from './Sidebar';
import Header from './Header';
import UpdateManager from './UpdateManager';
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
          <UpdateManager autoCheckOnStart={true} />
        </Content>
      </AntLayout>
    </AntLayout>
  );
};

export default Layout;