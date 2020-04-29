import AppHome from '@/components/AppHome';
const editor = () => import('@/view/editor.vue');


const routes = [
    {
        path: '/',
        name: 'Home',
        component: AppHome
    },
    {
        path: '/editor',
        name: 'editor',
        component: editor
    }
];

export default routes;
