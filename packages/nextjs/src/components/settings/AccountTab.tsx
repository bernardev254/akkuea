import { LogOutIcon, RefreshCcw, User, WalletIcon } from 'lucide-react';
import { Input } from '../ui/input';
import { Button } from '../ui/button';

export const AccountTab = () => {
  return (
    <div className="bg-white dark:bg-[#1F2937] rounded-xl p-4 sm:p-6 mb-6 shadow-sm border border-[#E5E7EB]">
      <div className=" mb-4 xl:mb-7">
        <div className="flex items-center gap-2 mb-2">
          <span className="text-[#00CED1] dark:text-teal-400">
            <User className="w-5 h-5" />
          </span>
          <h2 className="text-lg xl:text-2xl font-semibold">Account Information</h2>
        </div>
        <p className="text-sm text-white dark:text-[#71717A] mb-4">Manage your account details</p>
      </div>
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <div>
          <label htmlFor="email">Email</label>
          <div className="flex">
            <Input
              type="email"
              id="email"
              placeholder="Email"
              className="bg-white dark:bg-[#1F2937] border-[#E4E4E7] dark:border-white rounded-r-none"
            />
            <Button className="bg-transparent border-[#D1D5DB] h-9 border-2 dark:border-[#374151]  text-[#111827] dark:text-white mb-0.5 rounded-l-none">
              Verify
            </Button>
          </div>
        </div>
        <div>
          <label htmlFor="username">Username</label>
          <Input
            type="text"
            id="username"
            placeholder="Username"
            className="bg-white dark:bg-[#1F2937] border-white"
          />
        </div>
      </div>
      <div>
        <div className="flex items-center gap-2 mb-4 mt-12">
          <span className="text-[#00CED1] dark:text-teal-400">
            <WalletIcon className="w-5 h-5" />
          </span>
          <h2 className="font-medium">Connected Wallet</h2>
        </div>
        <div className="flex lg:items-center gap-2 mb-4 border-[#E5E7EB] dark:border-[#374151] bg-transparent border rounded-lg p-4 justify-between max-lg:flex-col max-lg:gap-8">
          <div>
            <p className="flex items-center gap-2">
              <span>GDUKIJ...ZNY</span>
              <span className="text-xs bg-[#EFF6FF] dark:bg-[#1E3A8A33] text-[#1D4ED8] border-[#BFDBFE] dark:text-[#60A5FA] dark:border-[#60A5FA] block px-2.5 py-1 rounded-full border">
                Stellar Testnet
              </span>
            </p>
            <p className="text-xs text-[#6B7280] dark:text-[#9CA3AF]">
              Stellar wallet connected successfully
            </p>
          </div>
          <div className="flex gap-2 items-center">
            <Button variant="outline" className="bg-transparent border-2 border-[#D1D5DB]">
              <RefreshCcw />
              Change Wallet
            </Button>
            <Button
              variant="destructive"
              className="bg-[#EF4444] hover:bg-[#DC2626]/80 dark:bg-transparent dark:hover:bg-[#DC2626]"
            >
              <LogOutIcon />
              Disconnect
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
};
