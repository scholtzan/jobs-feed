<div class="flex">
    <aside class="h-screen sticky top-0 flex flex-col bg-base-200 overflow-y-auto">
        <!-- Header -->
        <div class="flex flex-col justify-between p-2">
            <!-- Logo -->
            <a class="btn btn-ghost text-lg">
                Jobs Feed
            </a>

            <div>
                <a class="btn btn-ghost btn-circle text-lg">
                    <i class="fa-solid fa-gear"></i>
                </a>
            </div>

            <div>
                <!-- Search input -->
                <input class="input input-bordered" placeholder="Search..."/>
            </div>
        </div>

        <!-- Body -->
        <div class="flex flex-col border-y border-base-300 px-6 pt-4 grow">
            <!-- Links -->
            <div class="flex flex-col divide-y divide-base-300">
                <ul class="menu px-0 py-4">
                    <li>
                        <a>
                            <i class="fa-solid fa-house fa-fw"></i>
                            All Sources 
                        </a>
                    </li>
                    <li>
                        <a>
                            <i class="fa-solid fa-users fa-fw"></i>
                            Denny's 
                        </a>
                    </li>
                    <li>
                        <a class="active">
                            <i class="fa-solid fa-fire fa-fw"></i>
                            Earls 
                        </a>
                    </li>
                    <li>
                        <a>
                            <i class="fa-solid fa-podcast fa-fw"></i>
                            McDonald's 
                        </a>
                    </li>
                </ul>
            </div>
        </div>

        <!-- Footer -->
        <!--
        <div class="flex justify-between items-center p-2">
            <a class="btn">
                <img alt="Profile" src="/avatar.png" class="w-8 rounded-full" />

                <div class="flex flex-col text-start">
                    <span class="font-bold">User name</span>
                    <span class="text-sm text-accent">user@email.com</span>
                </div>
            </a>

            <a class="btn btn-error btn-sm mr-3" title="Logout">
                <i class="fa-solid fa-door-open"></i>
            </a>
        </div>
        -->
    </aside>
</div>