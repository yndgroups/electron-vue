import { createRouter, createWebHistory } from "vue-router";


const router = createRouter({ //设置为history模式
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: () => import('../views/Main.vue')
        },
        {
            path: '/test',
            component: () => import('../views/Test.vue')
        }
    ]
  })

  export default router
  