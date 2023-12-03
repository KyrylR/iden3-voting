import {
  createRouter,
  createWebHistory,
  RouteRecordRaw,
  useRoute,
  useRouter,
} from 'vue-router'

import { ROUTE_NAMES } from '@/enums'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/:catchAll(.*)',
    redirect: { name: ROUTE_NAMES.uiKit },
  },
  {
    path: '/ui-kit',
    name: ROUTE_NAMES.uiKit,
    component: () => import('@/pages/UiKitPage.vue'),
  },
  {
    path: '/complex-form',
    name: ROUTE_NAMES.complexForm,
    component: () => import('@/forms/ComplexForm.vue'),
  },
  {
    path: '/not-found',
    name: ROUTE_NAMES.notFound,
    component: () => import('@/pages/NotFoundPage.vue'),
  },
  {
    path: '/governance',
    name: ROUTE_NAMES.voting,
    component: () => import('@/pages/VotingPage.vue'),
  },
  {
    path: '/governance/create-proposal',
    name: ROUTE_NAMES.createProposal,
    component: () => import('@/pages/CreateProposalPage.vue'),
  },
  {
    path: '/governance/proposal/:id',
    name: ROUTE_NAMES.proposal,
    component: () => import('@/pages/VotingDetailsPage.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
  scrollBehavior: () => ({ top: 0, left: 0 }),
})

export { router, useRouter, useRoute }
