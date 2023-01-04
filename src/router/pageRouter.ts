export const pageRouter = [
  {
    path: '/home',
    name: 'Home',
    component: () => import('@/pages/Home/index.vue'),
    meta: {
      title: '首页'
    }
  },
  {
    path: '/list',
    name: 'List',
    component: () => import('@/pages/List/index.vue'),
    meta: {
      title: '患者列表'
    }
  },
  {
    path: '/archives',
    name: 'Archives',
    component: () => import('@/pages/Archives/index.vue'),
    meta: {
      title: '患者档案'
    }
  },
  {
    path: '/data',
    name: 'Data',
    component: () => import('@/pages/Data/index.vue'),
    meta: {
      title: '查看数据',
      keepAlive: true
    }
  },
  {
    path: '/report',
    name: 'Report',
    component: () => import('@/pages/Report/index.vue'),
    meta: {
      title: '报告'
    }
  },
  {
    path: '/setup',
    name: 'SetUp',
    component: () => import('@/pages/SetUp/index.vue'),
    meta: {
      title: '设置'
    }
  }
]