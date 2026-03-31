import { createBrowserRouter } from 'react-router-dom';
import Layout from '../components/Layout';
import Home from '../pages/Home';
import Download from '../pages/Download';
import Radar from '../pages/Radar';
import Settings from '../pages/Settings';
import PrivacySettings from '../pages/PrivacySettings';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout><Home /></Layout>,
  },
  {
    path: '/download',
    element: <Layout><Download /></Layout>,
  },
  {
    path: '/radar',
    element: <Layout><Radar /></Layout>,
  },
  {
    path: '/settings',
    element: <Layout><Settings /></Layout>,
  },
  {
    path: '/privacy',
    element: <Layout><PrivacySettings /></Layout>,
  },
]);

export default router;